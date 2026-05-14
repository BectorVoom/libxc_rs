//! MGGA_C_KCISK lxc pol — lxc_pol part 28 (v4rho3sigma_8) CSE chunk 1011/1456 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part28_v4rho3sigma_8_chunk1011<F: Float>(t1785: F, t8514: F, t5015: F, t17717: F, t22392: F, t17722: F, t22396: F, t22501: F, t7242: F, t22506: F, t7234: F, t8510: F, t10802: F, t10789: F, t10795: F, t10856: F, t17208: F, t5013: F, t7208: F, t7264: F, t8798: F) -> (F,) {
    let t23386 = t8514 * t1785;
    let t23387 = t5015 * t23386;
    let t23390 = t17717 * t22392;
    let t23393 = t17722 * t22396;
    let t23396 = t7242 * t22501;
    let t23399 = t7234 * t22506;
    let t23402 = t8510 * t1785;
    let t23403 = t10802 * t23402;
    let t23409 = -0.35981577432354634426e-1 * t10856 * t8798 + 0.35981577432354634426e-1 * t5013 * t23387 + 0.55971342672551653552e-1 * t5013 * t23390 - 0.95950873152945691804e-1 * t5013 * t23393 - 0.35981577432354634426e-1 * t5013 * t23396 + 0.23987718288236422951e-1 * t5013 * t23399 - 0.23987718288236422951e-1 * t5013 * t23403 - 0.11993859144118211475e-1 * t10789 + t10795 + t17208 + 0.21588946459412780656e0 * t7208 * t7264;
    (t23409,)
}
