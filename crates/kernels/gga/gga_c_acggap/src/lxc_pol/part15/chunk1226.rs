//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 1226/1278 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk1226<F: Float>(t34611: F, t34616: F, t34622: F, t34636: F, t34638: F, t34640: F, t34650: F, t37175: F, t37179: F, t37180: F, t37182: F, t37184: F, t39402: F, t39406: F, t39412: F, t39414: F, t39418: F, t39422: F) -> F {
    let t41638 = -t37175 + F::new(0.22921875e-1) * t39402 + F::new(0.25724410870841842184e-2) * t39406 - F::new(0.94344276868812456205e-2) * t34611 - F::new(0.37737710747524982482e-1) * t34616 - t37179 + t37180 - F::new(0.75475421495049964964e-2) * t34622 - t37182 - t37184 - F::new(0.18868855373762491241e-2) * t34636 + F::new(0.62896184579208304138e-3) * t34638 + F::new(0.56606566121287473723e-1) * t34640 - F::new(0.34299214494455789578e-2) * t39412 - F::new(0.34299214494455789578e-2) * t39414 + F::new(0.4584375e-1) * t34650 + F::new(0.18868855373762491241e-2) * t39418 - F::new(0.37737710747524982483e-2) * t39422;
    t41638
}
