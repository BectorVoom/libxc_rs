//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 32 (v4rho3sigma_8) CSE chunk 1625/2369 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1625<F: Float>(t1246: F, t19128: F, t5079: F, t6256: F, t3625: F, t5011: F, t1755: F, t5068: F, t1235: F, t6224: F, t1215: F, t475: F, t6739: F) -> (F, F, F, F, F, F, F, F) {
    let t19129 = t19128 * t1246;
    let t19131 = t6256 * t5079;
    let t19138 = t3625 * t5011;
    let t19139 = t1755 * t19138;
    let t19142 = t6256 * t5068;
    let t19145 = t1235 * t6224;
    let t19146 = t19145 * t3625;
    let t19153 = t6739 * t1215 * t475;
    (t19129, t19131, t19138, t19139, t19142, t19145, t19146, t19153)
}
