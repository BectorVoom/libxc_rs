//! GGA_C_GAPC lxc pol — lxc_pol part 27 (v4rho2sigma2_6) CSE chunk 396/1310 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part27_v4rho2sigma2_6_chunk396(t1403: f64, t681: f64, t201: f64, t197: f64, t1687: f64, t1823: f64, t1826: f64, t1830: f64, t1835: f64, t1842: f64, t1847: f64, t1850: f64, t1855: f64, t1861: f64, t1867: f64, t1872: f64, t1876: f64, t1878: f64, t1883: f64, t1888: f64, t1891: f64, t1896: f64, t1905: f64, t1912: f64, t192: f64, t578: f64, t582: f64, t585: f64, t590: f64, t591: f64, t605: f64, t625: f64, t634: f64, t637: f64, t642: f64, t656: f64) -> (f64, f64) {
    let t1915 = t681 * t1403;
    let t1916 = t201 * t1915;
    let t1917 = t197 * t1916;
    let t1920 = -0.6487109086417285278e-2_f64 * t578 * t605 - 0.69504740211613770836e-4_f64 * t590 * t1823 - 0.69504740211613770836e-4_f64 * t1826 * t591 + 0.2085142206348413125e-3_f64 * t1830 * t591 + 0.2318836277704281739e-4_f64 * t1687 * t1835 - 0.34782544165564226085e-4_f64 * t192 * t1842 + 0.11594181388521408695e-4_f64 * t1847 * t637 - 0.54106179813099907242e-4_f64 * t634 * t1850 - 0.10305939012019029951e-5_f64 * t642 * t1855 + 0.18323959563369835253e-5_f64 * t656 * t1855 - 0.27801896084645508334e-2_f64 * t1861 * t585 - 0.27801896084645508334e-2_f64 * t578 * t625 - 0.27801896084645508334e-2_f64 * t582 * t1867 - 0.13900948042322754167e-2_f64 * t582 * t1872 - 0.40544431790108032986e-3_f64 * t1876 * t1878 + 0.7324140771837707598e-5_f64 * t1883 * t1888 + 0.60073333102343402209e-5_f64 * t1891 * t1896 - 0.24657764237740843144e-6_f64 * t1905 * t1912 + 0.11594181388521408695e-4_f64 * t192 * t1917;
    (t1917, t1920)
}
