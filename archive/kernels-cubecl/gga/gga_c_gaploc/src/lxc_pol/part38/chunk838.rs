//! GGA_C_GAPLOC lxc pol — lxc_pol part 38 (v4rhosigma3_3) CSE chunk 838/1003 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part38_v4rhosigma3_3_chunk838<F: Float>(t44404: F, t6508: F, t42640: F, t42644: F, t42647: F, t1063: F, t13250: F, t1358: F, t2268: F, t2343: F, t2765: F, t34882: F, t44355: F, t44358: F, t44363: F, t44367: F, t44371: F, t44375: F, t44377: F, t44382: F, t44390: F, t44394: F, t44396: F, t44403: F, t6313: F, t6507: F, t7897: F) -> (F, F) {
    let t44405 = t6508 * t44404;
    let t44409 = F::cast_from(0.142275033178380748e-1_f64) * t42640;
    let t44410 = F::cast_from(0.33197507741622174533e-1_f64) * t42644;
    let t44411 = F::cast_from(0.56910013271352299199e-1_f64) * t42647;
    let t44412 = -t44355 - t44358 - F::cast_from(0.22764005308540919679e0_f64) * t6313 * t13250 + t44363 - t44367 + t44371 - t44375 + t44377 + F::cast_from(0.17073003981405689759e0_f64) * t1063 * t2765 * t34882 + F::cast_from(0.56910013271352299198e-1_f64) * t2268 * t2343 * t44382 + t44390 + t44394 - F::cast_from(0.63233348079280332442e-2_f64) * t1358 * t7897 * t44396 - t44403 - F::cast_from(0.12646669615856066489e-1_f64) * t1358 * t6507 * t44405 + t44409 - t44410 + t44411;
    (t44405, t44412)
}
