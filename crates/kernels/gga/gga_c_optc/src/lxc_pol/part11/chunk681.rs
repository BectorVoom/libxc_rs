//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 681/1451 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk681<F: Float>(t1825: F, t508: F, t1847: F, t1803: F, t1808: F, t1811: F, t1821: F, t1829: F, t1842: F, t1848: F, t1850: F, t1860: F, t1865: F, t1868: F, t209: F, t3648: F, t4: F, t573: F, t588: F, t6465: F, t6466: F, t6472: F, t6477: F, t6480: F, t6484: F, t6488: F, t6492: F, t6493: F, t6500: F, t6504: F, t71: F) -> (F, F, F) {
    let t6511 = t508 * t1825;
    let t6519 = t508 * t1847;
    let t6523 = -t6465 + F::cast_from(0.35089340384731224426e1_f64) * t1865 * t6466 + F::cast_from(0.16562449037037037036e-2_f64) * t4 * t3648 * t71 - F::cast_from(0.35089340384731224426e1_f64) * t1848 * t6472 - t6477 + t6480 + t6484 - t6488 - t6492 + F::cast_from(0.21687161765563048428e-1_f64) * t209 * t6493 * t588 - F::cast_from(0.16265371324172286321e-1_f64) * t209 * t1842 * t1860 - F::cast_from(0.48159446095139119799e0_f64) * t209 * t6500 * t1868 + F::cast_from(0.68493333333333333332e-1_f64) * t209 * t6504 * t573 - F::cast_from(0.51369999999999999999e-1_f64) * t209 * t1803 * t1821 - F::cast_from(0.16522997748472177549e1_f64) * t209 * t6511 * t1829 + F::new(0.10274e0) * t209 * t508 * t1808 * t1811 + F::cast_from(0.32530742648344572643e-1_f64) * t209 * t6519 * t1850;
    (t6511, t6519, t6523)
}
