//! GGA_C_GAPLOC lxc pol — lxc_pol part 33 (v4rho2sigma2_16) CSE chunk 392/1464 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part33_v4rho2sigma2_16_chunk392(t169: f64, t1865: f64, t299: f64, t706: f64, t1131: f64, t1713: f64, t1717: f64, t1820: f64, t1822: f64, t1827: f64, t1837: f64, t1841: f64, t1846: f64, t1850: f64, t1855: f64, t1860: f64, t270: f64, t650: f64, t681: f64, t703: f64, t726: f64, t741: f64) -> (f64, f64) {
    let t1866 = t1865 * t169;
    let t1868 = t706 * t1866 * t299;
    let t1875 = 0.76905262301422242837e-2_f64 * t270 * t1713 - 0.76905262301422242837e-2_f64 * t270 * t1717 + 0.20508069947045931424e-1_f64 * t650 * t726 - 0.15381052460284448567e-1_f64 * t681 * t703 + 0.15381052460284448567e-1_f64 * t681 * t726 - 0.76905262301422242837e-2_f64 * t270 * t1837 + 0.17090058289204942853e-2_f64 * t1841 * t1846 + 0.17090058289204942853e-2_f64 * t1850 * t1855 - 0.17090058289204942853e-2_f64 * t1841 * t1860 + t1820 + t1822 + t1827 - t1131 - 0.20508069947045931424e-1_f64 * t650 * t703 + 0.76905262301422242837e-2_f64 * t270 * t1868 - 0.20508069947045931424e-1_f64 * t650 * t741 - 0.15381052460284448567e-1_f64 * t681 * t741;
    (t1866, t1875)
}
