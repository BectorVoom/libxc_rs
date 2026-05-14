//! GGA_C_GAPLOC lxc pol — lxc_pol part 30 (v4rho2sigma2_13) CSE chunk 918/1268 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part30_v4rho2sigma2_13_chunk918<F: Float>(t10648: F, t10711: F, t10748: F, t10793: F, t8440: F, t977: F, t2728: F, t2969: F, t3455: F, t747: F, t3459: F, t841: F, t5559: F, t1052: F, t1960: F, t10284: F, t10287: F, t10291: F, t10292: F, t10294: F, t10303: F, t10304: F, t10307: F, t1955: F, t331: F, t3511: F) -> (F, F, F, F, F) {
    let t10795 = t10648 + t10711 + t10748 + t10793;
    let t10797 = t8440 * t977;
    let t10798 = t2969 * t2728;
    let t10800 = t3455 * t747;
    let t10802 = t3459 * t841;
    let t10804 = 6.0 * t5559 * t10802;
    let t10805 = t1052 * t2728;
    let t10807 = 2.0 * t1960 * t10805;
    let t10808 = t10795 * t331 - t10800 * t841 - t1955 * t3511 + t10284 - t10287 - t10291 + t10292 - t10294 - t10303 + t10304 - t10307 - t10797 - t10798 - t10804 + t10807;
    (t10795, t10800, t10802, t10805, t10808)
}
