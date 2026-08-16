//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 688/1414 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk688(t1825: f64, t508: f64, t1847: f64, t1803: f64, t1808: f64, t1811: f64, t1821: f64, t1829: f64, t1842: f64, t1848: f64, t1850: f64, t1860: f64, t1865: f64, t1868: f64, t209: f64, t3648: f64, t4: f64, t573: f64, t588: f64, t6465: f64, t6466: f64, t6472: f64, t6477: f64, t6480: f64, t6484: f64, t6488: f64, t6492: f64, t6493: f64, t6500: f64, t6504: f64, t71: f64) -> (f64, f64, f64) {
    let t6511 = t508 * t1825;
    let t6519 = t508 * t1847;
    let t6523 = -t6465 + 0.35089340384731224426e1_f64 * t1865 * t6466 + 0.16562449037037037036e-2_f64 * t4 * t3648 * t71 - 0.35089340384731224426e1_f64 * t1848 * t6472 - t6477 + t6480 + t6484 - t6488 - t6492 + 0.21687161765563048428e-1_f64 * t209 * t6493 * t588 - 0.16265371324172286321e-1_f64 * t209 * t1842 * t1860 - 0.48159446095139119799e0_f64 * t209 * t6500 * t1868 + 0.68493333333333333332e-1_f64 * t209 * t6504 * t573 - 0.51369999999999999999e-1_f64 * t209 * t1803 * t1821 - 0.16522997748472177549e1_f64 * t209 * t6511 * t1829 + 0.10274e0_f64 * t209 * t508 * t1808 * t1811 + 0.32530742648344572643e-1_f64 * t209 * t6519 * t1850;
    (t6511, t6519, t6523)
}
