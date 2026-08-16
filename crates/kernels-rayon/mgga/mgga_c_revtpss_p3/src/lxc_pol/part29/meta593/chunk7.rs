//! MGGA_C_REVTPSS lxc pol — lxc_pol part 29 (v4rho3sigma_4) CSE chunk 1984/2049 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1984(t102480: f64, t102493: f64, t102507: f64, t102519: f64, t102533: f64, t102546: f64, t102558: f64, t102570: f64, t1904: f64, t2439: f64, t26358: f64, t102453: f64, t102458: f64, t102462: f64, t102465: f64, t14224: f64, t213: f64, t225: f64, t25921: f64, t25924: f64, t25930: f64, t26304: f64, t27868: f64, t28841: f64, t4077: f64, t49306: f64, t561: f64, t7295: f64, t8085: f64, t96392: f64, t96456: f64, t96458: f64, t96460: f64, t96464: f64, t97858: f64) -> (f64, f64) {
    let t102573 = t102480 + t102493 + t102507 + t102519 + t102533 + t102546 + t102558 + t102570;
    let t102582 = t2439 * t26358 * t1904;
    let t102584 = 0.8673628188205199462e0_f64 * t27868 * t96392 * t14224 - 0.26020884564615598386e1_f64 * t7295 * t25924 * t8085 * t4077 + 0.91399340044406952588e-2_f64 * t96456 - t102453 + 0.4336814094102599731e0_f64 * t27868 * t26304 * t49306 - t102458 + 0.17347256376410398924e1_f64 * t25921 * t28841 - 0.28912093960683998208e-1_f64 * t96458 + 0.73171657588172351096e-2_f64 * t102462 - t102465 + 0.2601984143835408805e-1_f64 * t96460 + 0.19514881078765566038e-1_f64 * t96464 + 0.65854491829355115987e0_f64 * t213 * t102573 * t225 * t561 - 0.8673628188205199462e0_f64 * t25930 * t26304 * t97858 + 0.65049603595885220126e-3_f64 * t102582;
    (t102573, t102584)
}
