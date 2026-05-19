//! GGA_C_GAPLOC lxc pol — lxc_pol part 18 (v4rho2sigma2_1) CSE chunk 393/1436 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part18_v4rho2sigma2_1_chunk393<F: Float>(t169: F, t1865: F, t299: F, t706: F, t1131: F, t1713: F, t1717: F, t1820: F, t1822: F, t1827: F, t1837: F, t1841: F, t1846: F, t1850: F, t1855: F, t1860: F, t270: F, t650: F, t681: F, t703: F, t726: F, t741: F) -> (F, F) {
    let t1866 = t1865 * t169;
    let t1868 = t706 * t1866 * t299;
    let t1875 = F::cast_from(0.76905262301422242837e-2_f64) * t270 * t1713 - F::cast_from(0.76905262301422242837e-2_f64) * t270 * t1717 + F::cast_from(0.20508069947045931424e-1_f64) * t650 * t726 - F::cast_from(0.15381052460284448567e-1_f64) * t681 * t703 + F::cast_from(0.15381052460284448567e-1_f64) * t681 * t726 - F::cast_from(0.76905262301422242837e-2_f64) * t270 * t1837 + F::cast_from(0.17090058289204942853e-2_f64) * t1841 * t1846 + F::cast_from(0.17090058289204942853e-2_f64) * t1850 * t1855 - F::cast_from(0.17090058289204942853e-2_f64) * t1841 * t1860 + t1820 + t1822 + t1827 - t1131 - F::cast_from(0.20508069947045931424e-1_f64) * t650 * t703 + F::cast_from(0.76905262301422242837e-2_f64) * t270 * t1868 - F::cast_from(0.20508069947045931424e-1_f64) * t650 * t741 - F::cast_from(0.15381052460284448567e-1_f64) * t681 * t741;
    (t1866, t1875)
}
