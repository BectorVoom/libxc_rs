//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 1317/1336 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk1317<F: Float>(t1167: F, t179: F, t19150: F, t404: F, t10213: F, t1238: F, t19039: F, t19055: F, t19067: F, t19070: F, t19073: F, t19076: F, t19099: F, t19102: F, t19124: F, t19128: F, t19166: F, t23022: F, t23248: F, t23250: F, t23254: F, t23264: F, t23266: F, t2380: F, t2381: F, t3206: F, t6408: F, t6416: F, t6483: F, t6518: F, t8254: F, t8428: F) -> F {
    let t23272 = t404 * t179 * t19150 * t1167;
    let t23275 = -F::cast_from(0.25724410870841842183e-2_f64) * t8428 * t2381 * t23022 * t6518 + F::cast_from(0.12862205435420921092e-2_f64) * t3206 * t8254 * t6416 * t6483 + F::cast_from(0.38586616306262763276e-2_f64) * t2380 * t10213 * t19166 - F::cast_from(0.22866142996303859718e-2_f64) * t23248 - F::cast_from(0.45732285992607719436e-2_f64) * t23250 + F::cast_from(0.51448821741683684367e-2_f64) * t23254 + t19039 / F::new(48.0) - t19055 - F::cast_from(0.14291339372689912324e-3_f64) * t19067 + F::cast_from(0.85748036236139473944e-3_f64) * t19070 - F::cast_from(0.85748036236139473944e-3_f64) * t19073 + F::cast_from(0.14291339372689912324e-3_f64) * t19076 - F::cast_from(0.28582678745379824648e-3_f64) * t19099 + F::cast_from(0.14291339372689912324e-3_f64) * t19102 + F::cast_from(0.19055119163586549765e-3_f64) * t19124 + F::cast_from(0.22866142996303859718e-2_f64) * t23264 - F::cast_from(0.13719685797782315831e-1_f64) * t23266 + F::cast_from(0.27439371595564631662e-1_f64) * t1238 * t6408 - F::cast_from(0.1270341277572436651e-3_f64) * t23272 + F::cast_from(0.28582678745379824648e-3_f64) * t19128;
    t23275
}
