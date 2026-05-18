//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1133/1478 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1133<F: Float>(t301: F, t413: F, t5988: F, t5980: F, t76: F, t2209: F, t11567: F, t11569: F, t1227: F, t1233: F, t1309: F, t1316: F, t14596: F, t14617: F, t14648: F, t14656: F, t14786: F, t2180: F, t2236: F, t2255: F, t2258: F, t2308: F, t2733: F, t342: F, t346: F, t374: F, t384: F, t4042: F, t4398: F, t5583: F, t5829: F, t5934: F, t5992: F, t6006: F, t6007: F, t6008: F, t6024: F, t73: F, t77: F, t790: F) -> (F, F) {
    let t14789 = t5988 * t413 * t301;
    let t14797 = t76 * t5980;
    let t14816 = t2209 * t2209;
    let t14831 = F::new(6.0) * t1316 * t790 * t14656 + F::new(12.0) * t1316 * t790 * t14617 - F::new(0.0005811348303577384) * t14786 - F::new(0.0005811348303577384) * t14789 + F::new(6.0) * t1316 * t2258 * t6024 + F::new(2.0) * t346 * t2258 * t2255 + F::new(12.0) * t2180 * t14797 * t342 + F::new(6.0) * t2180 * t5992 * t1227 + F::new(4.0) * t6006 * t6007 * t2236 * t374 + F::new(4.0) * t6006 * t4042 * t384 * t6008 - F::new(0.21287527129341527) * t11567 - F::new(0.31931290694012293) * t11569 - F::new(12.0) * t5583 * t14596 + F::new(12.0) * t1233 * t77 * t14816 - F::new(2.0) * t346 * t4398 * t5934 - t346 * t2308 * t73 * t5829 + F::new(6.0) * t1316 * t790 * t14648 + t346 * t2733 * t1309;
    (t14816, t14831)
}
