//! MGGA_C_KCISK lxc pol — lxc_pol part 23 (v4rho3sigma_3) CSE chunk 1340/1447 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part23_v4rho3sigma_3_chunk1340<F: Float>(t109294: F, t6373: F, t1415: F, t21006: F, t109311: F, t9839: F, t500: F, t52017: F, t33643: F, t4205: F, t3512: F, t6363: F, t32255: F, t6357: F, t32269: F, t6313: F) -> (F, F, F, F, F, F, F, F) {
    let t113506 = t109294 * t6373;
    let t113508 = t1415 * t21006;
    let t113511 = t109311 * t9839;
    let t113513 = t52017 * t500;
    let t113515 = t33643 * t4205;
    let t113517 = t3512 * t6363;
    let t113519 = t32255 * t6357;
    let t113521 = t32269 * t6313;
    (t113506, t113508, t113511, t113513, t113515, t113517, t113519, t113521)
}
