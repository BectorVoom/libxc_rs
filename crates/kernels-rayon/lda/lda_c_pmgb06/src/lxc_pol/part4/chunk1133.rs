//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1133/1478 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1133(t301: f64, t413: f64, t5988: f64, t5980: f64, t76: f64, t2209: f64, t11567: f64, t11569: f64, t1227: f64, t1233: f64, t1309: f64, t1316: f64, t14596: f64, t14617: f64, t14648: f64, t14656: f64, t14786: f64, t2180: f64, t2236: f64, t2255: f64, t2258: f64, t2308: f64, t2733: f64, t342: f64, t346: f64, t374: f64, t384: f64, t4042: f64, t4398: f64, t5583: f64, t5829: f64, t5934: f64, t5992: f64, t6006: f64, t6007: f64, t6008: f64, t6024: f64, t73: f64, t77: f64, t790: f64) -> (f64, f64) {
    let t14789 = t5988 * t413 * t301;
    let t14797 = t76 * t5980;
    let t14816 = t2209 * t2209;
    let t14831 = 6.0_f64 * t1316 * t790 * t14656 + 12.0_f64 * t1316 * t790 * t14617 - 0.0005811348303577384_f64 * t14786 - 0.0005811348303577384_f64 * t14789 + 6.0_f64 * t1316 * t2258 * t6024 + 2.0_f64 * t346 * t2258 * t2255 + 12.0_f64 * t2180 * t14797 * t342 + 6.0_f64 * t2180 * t5992 * t1227 + 4.0_f64 * t6006 * t6007 * t2236 * t374 + 4.0_f64 * t6006 * t4042 * t384 * t6008 - 0.21287527129341527_f64 * t11567 - 0.31931290694012293_f64 * t11569 - 12.0_f64 * t5583 * t14596 + 12.0_f64 * t1233 * t77 * t14816 - 2.0_f64 * t346 * t4398 * t5934 - t346 * t2308 * t73 * t5829 + 6.0_f64 * t1316 * t790 * t14648 + t346 * t2733 * t1309;
    (t14816, t14831)
}
