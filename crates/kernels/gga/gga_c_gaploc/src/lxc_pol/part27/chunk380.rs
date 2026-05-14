//! GGA_C_GAPLOC lxc pol — lxc_pol part 27 (v4rho2sigma2_10) CSE chunk 380/1296 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part27_v4rho2sigma2_10_chunk380<F: Float>(t169: F, t1865: F, t299: F, t706: F, t1131: F, t1713: F, t1717: F, t1820: F, t1822: F, t1827: F, t1837: F, t1841: F, t1846: F, t1850: F, t1855: F, t1860: F, t270: F, t650: F, t681: F, t703: F, t726: F, t741: F) -> (F, F) {
    let t1866 = t1865 * t169;
    let t1868 = t706 * t1866 * t299;
    let t1875 = 0.76905262301422242837e-2 * t270 * t1713 - 0.76905262301422242837e-2 * t270 * t1717 + 0.20508069947045931424e-1 * t650 * t726 - 0.15381052460284448567e-1 * t681 * t703 + 0.15381052460284448567e-1 * t681 * t726 - 0.76905262301422242837e-2 * t270 * t1837 + 0.17090058289204942853e-2 * t1841 * t1846 + 0.17090058289204942853e-2 * t1850 * t1855 - 0.17090058289204942853e-2 * t1841 * t1860 + t1820 + t1822 + t1827 - t1131 - 0.20508069947045931424e-1 * t650 * t703 + 0.76905262301422242837e-2 * t270 * t1868 - 0.20508069947045931424e-1 * t650 * t741 - 0.15381052460284448567e-1 * t681 * t741;
    (t1866, t1875)
}
