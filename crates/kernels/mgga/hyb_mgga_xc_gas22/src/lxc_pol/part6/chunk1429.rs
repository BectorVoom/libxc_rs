//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1429/1455 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1429<F: Float>(t2876: F, t4501: F, t2851: F, t11319: F, t7785: F, t11329: F, t11383: F, t11386: F, t11392: F, t14815: F, t22531: F, t26194: F, t26525: F, t2821: F, t2834: F, t30710: F, t30723: F, t30930: F, t30933: F, t30936: F, t3733: F, t3757: F, t7637: F, t7811: F, t9490: F) -> (F, F, F) {
    let t30955 = t4501 * t2876;
    let t30956 = t2851 * t30955;
    let t30961 = t11319 * t7785;
    let t30968 = -F::new(400.0) / F::new(9.0) * t26525 * t11392 + F::new(64.0) / F::new(9.0) * t26194 * t11386 + F::new(256.0) / F::new(81.0) * t22531 * t30933 + F::new(128.0) / F::new(27.0) * t7811 * t30936 + F::new(400.0) / F::new(9.0) * t26525 * t11383 - F::new(1280.0) / F::new(81.0) * t3757 * t30930 + F::new(616.0) / F::new(9.0) * t7637 * t30710 - F::new(64.0) / F::new(27.0) * t14815 * t30956 + F::new(800.0) / F::new(27.0) * t11329 * t9490 + F::new(88.0) / F::new(27.0) * t2821 * t30961 - F::new(3200.0) / F::new(81.0) * t3733 * t30723 + F::new(88.0) / F::new(9.0) * t2834 * t30961;
    (t30955, t30956, t30968)
}
