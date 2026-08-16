//! MGGA_C_REVTPSS lxc pol — lxc_pol part 29 (v4rho3sigma_4) CSE chunk 1984/2049 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1984<F: Float>(t102480: F, t102493: F, t102507: F, t102519: F, t102533: F, t102546: F, t102558: F, t102570: F, t1904: F, t2439: F, t26358: F, t102453: F, t102458: F, t102462: F, t102465: F, t14224: F, t213: F, t225: F, t25921: F, t25924: F, t25930: F, t26304: F, t27868: F, t28841: F, t4077: F, t49306: F, t561: F, t7295: F, t8085: F, t96392: F, t96456: F, t96458: F, t96460: F, t96464: F, t97858: F) -> (F, F) {
    let t102573 = t102480 + t102493 + t102507 + t102519 + t102533 + t102546 + t102558 + t102570;
    let t102582 = t2439 * t26358 * t1904;
    let t102584 = F::cast_from(0.8673628188205199462e0_f64) * t27868 * t96392 * t14224 - F::cast_from(0.26020884564615598386e1_f64) * t7295 * t25924 * t8085 * t4077 + F::cast_from(0.91399340044406952588e-2_f64) * t96456 - t102453 + F::cast_from(0.4336814094102599731e0_f64) * t27868 * t26304 * t49306 - t102458 + F::cast_from(0.17347256376410398924e1_f64) * t25921 * t28841 - F::cast_from(0.28912093960683998208e-1_f64) * t96458 + F::cast_from(0.73171657588172351096e-2_f64) * t102462 - t102465 + F::cast_from(0.2601984143835408805e-1_f64) * t96460 + F::cast_from(0.19514881078765566038e-1_f64) * t96464 + F::cast_from(0.65854491829355115987e0_f64) * t213 * t102573 * t225 * t561 - F::cast_from(0.8673628188205199462e0_f64) * t25930 * t26304 * t97858 + F::cast_from(0.65049603595885220126e-3_f64) * t102582;
    (t102573, t102584)
}
