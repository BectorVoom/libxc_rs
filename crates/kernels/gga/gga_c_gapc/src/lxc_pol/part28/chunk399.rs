//! GGA_C_GAPC lxc pol — lxc_pol part 28 (v4rho2sigma2_7) CSE chunk 399/1429 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part28_v4rho2sigma2_7_chunk399<F: Float>(t1403: F, t681: F, t201: F, t197: F, t1687: F, t1823: F, t1826: F, t1830: F, t1835: F, t1842: F, t1847: F, t1850: F, t1855: F, t1861: F, t1867: F, t1872: F, t1876: F, t1878: F, t1883: F, t1888: F, t1891: F, t1896: F, t1905: F, t1912: F, t192: F, t578: F, t582: F, t585: F, t590: F, t591: F, t605: F, t625: F, t634: F, t637: F, t642: F, t656: F) -> (F, F) {
    let t1915 = t681 * t1403;
    let t1916 = t201 * t1915;
    let t1917 = t197 * t1916;
    let t1920 = -F::new(0.6487109086417285278e-2) * t578 * t605 - F::new(0.69504740211613770836e-4) * t590 * t1823 - F::new(0.69504740211613770836e-4) * t1826 * t591 + F::new(0.2085142206348413125e-3) * t1830 * t591 + F::new(0.2318836277704281739e-4) * t1687 * t1835 - F::new(0.34782544165564226085e-4) * t192 * t1842 + F::new(0.11594181388521408695e-4) * t1847 * t637 - F::new(0.54106179813099907242e-4) * t634 * t1850 - F::new(0.10305939012019029951e-5) * t642 * t1855 + F::new(0.18323959563369835253e-5) * t656 * t1855 - F::new(0.27801896084645508334e-2) * t1861 * t585 - F::new(0.27801896084645508334e-2) * t578 * t625 - F::new(0.27801896084645508334e-2) * t582 * t1867 - F::new(0.13900948042322754167e-2) * t582 * t1872 - F::new(0.40544431790108032986e-3) * t1876 * t1878 + F::new(0.7324140771837707598e-5) * t1883 * t1888 + F::new(0.60073333102343402209e-5) * t1891 * t1896 - F::new(0.24657764237740843144e-6) * t1905 * t1912 + F::new(0.11594181388521408695e-4) * t192 * t1917;
    (t1917, t1920)
}
