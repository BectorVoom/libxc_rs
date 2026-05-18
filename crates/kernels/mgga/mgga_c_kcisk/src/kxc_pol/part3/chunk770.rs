//! MGGA_C_KCISK kxc pol — kxc_pol part 3 (v3rho3_0) CSE chunk 770/1063 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_kxc_pol_part3_v3rho3_0_chunk770<F: Float>(t4265: F, t5251: F, t10487: F, t702: F, t10441: F, t5248: F, t1919: F, t3293: F, t5254: F, t10449: F, t1920: F, t5261: F) -> (F, F, F, F, F) {
    let t11830 = t4265 * t5251;
    let t11832 = t702 * t10487;
    let t11834 = t5248 * t11832 * t10441;
    let t11838 = t1919 * t5254 * t3293;
    let t11842 = t1919 * t1920 * t10449;
    let t11851 = t4265 * t5261;
    (t11830, t11834, t11838, t11842, t11851)
}
