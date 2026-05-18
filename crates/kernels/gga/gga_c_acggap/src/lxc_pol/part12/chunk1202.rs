//! GGA_C_ACGGAP lxc pol — lxc_pol part 12 (v4rho3sigma_4) CSE chunk 1202/1250 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part12_v4rho3sigma_4_chunk1202<F: Float>(t35672: F, t35678: F, t35682: F, t35685: F, t35702: F, t31482: F, t31484: F, t31487: F, t31489: F, t31492: F, t35668: F, t35670: F, t35674: F, t35676: F, t35691: F, t35695: F, t35698: F, t35706: F) -> F {
    let t37658 = F::new(0.13719685797782315831e-1) * t35672;
    let t37661 = F::new(0.13719685797782315831e-1) * t35678;
    let t37663 = F::new(0.57165357490759649296e-3) * t35682;
    let t37665 = F::new(11.0) / F::new(24.0) * t35685;
    let t37672 = F::new(0.18868855373762491241e-2) * t35702;
    let t37674 = F::new(0.34299214494455789578e-1) * t35668 + F::new(0.17149607247227894789e-1) * t35670 - t37658 - F::new(0.13719685797782315831e-1) * t35674 - F::new(0.68598428988911579156e-2) * t35676 + t37661 - F::new(0.57165357490759649296e-3) * t31482 - t37663 - F::new(0.51448821741683684367e-2) * t31484 - t37665 + t31487 / F::new(48.0) - F::new(0.916875e-1) * t31489 - F::new(0.183375e0) * t31492 + F::new(0.18868855373762491241e-1) * t35691 - F::new(0.62896184579208304136e-2) * t35695 + F::new(0.94344276868812456204e-2) * t35698 + t37672 + F::new(0.85748036236139473944e-3) * t35706;
    t37674
}
