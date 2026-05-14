//! MGGA_C_KCISK lxc pol — lxc_pol part 28 (v4rho3sigma_8) CSE chunk 699/1456 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part28_v4rho3sigma_8_chunk699<F: Float>(t1835: F, t8518: F, t706: F, t8510: F, t8514: F, t1842: F, t158: F, t165: F, t1809: F, t1850: F, t5089: F, t6903: F, t6906: F, t6922: F, t6924: F, t6926: F, t7715: F, t7718: F) -> (F, F, F, F, F) {
    let t8640 = t1835 * t8518;
    let t8643 = t706 * t8510;
    let t8649 = t1835 * t8514;
    let t8652 = t1842 * t8514;
    let t8661 = -0.23911438650126355246e-1 * t5089 * t7715 - 0.3513e-2 * t158 * t8640 + 0.1171e-2 * t158 * t8643 + 0.9368e-2 * t6922 - 0.26416666666666666666e-2 * t6924 - 0.23526125e-4 * t6926 + 0.7026e-2 * t158 * t8649 - 0.1585e-2 * t165 * t8652 - 0.23911438650126355246e-1 * t6903 + 0.20718155631185227504e-3 * t6906 + 0.11955719325063177623e-1 * t1809 * t7718 - 0.5179538907796306876e-4 * t1850 * t7718;
    (t8640, t8643, t8649, t8652, t8661)
}
