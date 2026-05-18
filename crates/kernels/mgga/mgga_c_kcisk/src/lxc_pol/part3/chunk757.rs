//! MGGA_C_KCISK lxc pol — lxc_pol part 3 (v3rho3_0) CSE chunk 757/1063 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part3_v3rho3_0_chunk757<F: Float>(t11676: F, t11677: F, t5192: F, t5182: F, t1849: F, t642: F, t11219: F, t10504: F, t10777: F, t11201: F, t11239: F, t11241: F, t11245: F, t11453: F, t11456: F, t11650: F, t11652: F, t11661: F, t11663: F, t11669: F, t11674: F, t1693: F, t4823: F, t4827: F, t4830: F, t671: F) -> (F, F, F) {
    let t11678 = t11676 * t11677;
    let t11679 = t5192 * t11678;
    let t11680 = t5182 * t11679;
    let t11682 = t642 * t1849;
    let t11683 = t11682 * t11219;
    let t11684 = t5192 * t11683;
    let t11685 = t5182 * t11684;
    let t11687 = F::new(0.1492375e-1) * t11239 + F::new(0.99491666666666666664e-2) * t11241 - F::new(0.386e0) * t1693 * t11201 + F::new(0.223494e0) * t11245 * t4827 - F::new(0.24872916666666666666e-2) * t11453 + F::new(0.49745833333333333332e-2) * t11456 + F::new(0.24872916666666666666e-2) * t11650 + F::new(0.49745833333333333332e-2) * t11652 + F::new(0.579e0) * t4830 * t4827 - F::new(0.223494e0) * t4823 * t11201 - F::new(0.74618749999999999998e-2) * t11661 - F::new(0.99491666666666666664e-2) * t11663 + t10777 * t671 + F::new(0.223494e0) * t4823 * t10504 - F::new(0.99491666666666666664e-2) * t11669 + F::new(0.1492375e-1) * t11674 - F::new(0.11054629629629629629e-2) * t11680 - F::new(0.66327777777777777775e-2) * t11685;
    (t11680, t11685, t11687)
}
