//! MGGA_C_R2SCAN lxc pol — lxc_pol part 16 (v4rho3sigma_6) CSE chunk 515/1264 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part16_v4rho3sigma_6_chunk515<F: Float>(t2482: F, t595: F, t584: F, t591: F, t956: F, t170: F, t2461: F, t159: F, t1655: F, t1803: F, t1806: F, t1808: F, t1812: F, t1816: F, t1825: F, t1829: F, t1833: F, t1840: F, t1844: F, t1845: F, t1847: F, t1851: F, t598: F, t951: F) -> (F, F, F, F) {
    let t2774 = t595 * t2482;
    let t2780 = t584 * t956 * t591;
    let t2782 = t2461 * t170;
    let t2787 = -F::cast_from(0.675260332e-1_f64) * t2774 * t598 - F::cast_from(0.675260332e-1_f64) * t951 * t1655 - F::cast_from(0.571528e-1_f64) * t2780 - t1803 - t1806 + F::cast_from(0.285764e-1_f64) * t159 * t2782 - t1808 + t1812 + t1816 + t1825 - t1829 - t1833 - t1840 + t1844 - F::cast_from(0.11696447245269292414e1_f64) * t1845 + F::cast_from(0.17315859105681463759e2_f64) * t1847 + t1851;
    (t2774, t2780, t2782, t2787)
}
