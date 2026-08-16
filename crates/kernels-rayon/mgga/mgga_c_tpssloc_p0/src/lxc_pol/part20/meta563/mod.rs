//! MGGA_C_TPSSLOC lxc pol kernel — _part20_v4rho4_1 meta563 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2121;
use chunk1::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2122;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_meta563(t10403: f64, t10404: f64, t10422: f64, t10477: f64, t67: f64, t3067: f64, t11059: f64, t10970: f64, t820: f64, t10418: f64, t3070: f64, t10397: f64, t10517: f64, t3103: f64, t1041: f64, t10868: f64, t248: f64, t2780: f64, t10316: f64, t3051: f64, t10277: f64, t976: f64, t10993: f64, t2960: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t42380, t42386, t42387, t42388, t42397, t42403, t42412) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2121(t10403, t10404, t10422, t10477, t67, t3067, t11059, t10970, t820, t10418, t3070, t10397);
        let (t42428, t42432, t42436, t42444, t42460) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2122(t10517, t3103, t1041, t10868, t248, t2780, t10316, t3051, t10277, t976, t10993, t2960);
    (t42380, t42386, t42387, t42388, t42397, t42403, t42412, t42428, t42432, t42436, t42444, t42460)
}
