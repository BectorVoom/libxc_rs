//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 1239/1278 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk1239<F: Float>(t35260: F, t35271: F, t37458: F, t39797: F, t39802: F, t39807: F, t39809: F, t39811: F, t39813: F, t39815: F, t39817: F, t39819: F, t39822: F, t39825: F, t39829: F, t39831: F, t39833: F, t39835: F) -> F {
    let t41815 = t37458 + F::new(0.12862205435420921092e-2) * t39797 - F::new(0.75475421495049964965e-2) * t35260 - F::new(0.62896184579208304137e-2) * t39802 - F::new(0.37737710747524982482e-1) * t39807 - F::new(0.34299214494455789578e-2) * t39809 - F::new(0.13719685797782315831e-1) * t39811 + F::new(0.68598428988911579156e-2) * t39813 - F::new(0.10289764348336736873e-1) * t39815 + F::new(0.34299214494455789578e-2) * t39817 + F::new(0.51448821741683684367e-2) * t39819 + t39822 / F::new(12.0) + t39825 / F::new(96.0) + F::new(0.94344276868812456205e-2) * t39829 - F::new(0.34299214494455789578e-2) * t39831 + F::new(0.17149607247227894789e-2) * t39833 + F::new(0.32012600194825403606e-1) * t39835 - F::new(0.42874018118069736972e-3) * t35271;
    t41815
}
