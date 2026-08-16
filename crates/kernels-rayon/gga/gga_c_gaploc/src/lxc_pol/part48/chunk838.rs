//! GGA_C_GAPLOC lxc pol — lxc_pol part 48 (v4rhosigma3_13) CSE chunk 838/1003 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part48_v4rhosigma3_13_chunk838(t44404: f64, t6508: f64, t42640: f64, t42644: f64, t42647: f64, t1063: f64, t13250: f64, t1358: f64, t2268: f64, t2343: f64, t2765: f64, t34882: f64, t44355: f64, t44358: f64, t44363: f64, t44367: f64, t44371: f64, t44375: f64, t44377: f64, t44382: f64, t44390: f64, t44394: f64, t44396: f64, t44403: f64, t6313: f64, t6507: f64, t7897: f64) -> (f64, f64) {
    let t44405 = t6508 * t44404;
    let t44409 = 0.142275033178380748e-1_f64 * t42640;
    let t44410 = 0.33197507741622174533e-1_f64 * t42644;
    let t44411 = 0.56910013271352299199e-1_f64 * t42647;
    let t44412 = -t44355 - t44358 - 0.22764005308540919679e0_f64 * t6313 * t13250 + t44363 - t44367 + t44371 - t44375 + t44377 + 0.17073003981405689759e0_f64 * t1063 * t2765 * t34882 + 0.56910013271352299198e-1_f64 * t2268 * t2343 * t44382 + t44390 + t44394 - 0.63233348079280332442e-2_f64 * t1358 * t7897 * t44396 - t44403 - 0.12646669615856066489e-1_f64 * t1358 * t6507 * t44405 + t44409 - t44410 + t44411;
    (t44405, t44412)
}
