//! MGGA_C_KCISK lxc pol — lxc_pol part 23 (v4rho3sigma_3) CSE chunk 1226/1447 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part23_v4rho3sigma_3_chunk1226<F: Float>(t32278: F, t6373: F, t32287: F, t9839: F, t33644: F, t33646: F, t33648: F, t33650: F, t33653: F, t33656: F, t33659: F, t33661: F, t33663: F, t33665: F, t33667: F, t1517: F, t5606: F) -> (F, F, F, F) {
    let t33669 = t32278 * t6373;
    let t33671 = t32287 * t9839;
    let t33673 = -t33644 / 16.0 + t33646 / 96.0 - t33648 / 128.0 + t33650 / 16.0 - t33653 / 64.0 - t33656 / 288.0 - t33659 / 16.0 - t33661 / 16.0 - t33663 / 6.0 + t33665 / 18.0 + t33667 / 128.0 + t33669 / 96.0 - t33671 / 9.0;
    let t33674 = t5606 * t1517;
    (t33669, t33671, t33673, t33674)
}
