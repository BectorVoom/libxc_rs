//! MGGA_C_R2SCAN lxc pol — lxc_pol part 15 (v4rho3sigma_5) CSE chunk 567/1253 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part15_v4rho3sigma_5_chunk567(t2482: f64, t595: f64, t584: f64, t591: f64, t956: f64, t170: f64, t2461: f64, t159: f64, t1655: f64, t1803: f64, t1806: f64, t1808: f64, t1812: f64, t1816: f64, t1825: f64, t1829: f64, t1833: f64, t1840: f64, t1844: f64, t1845: f64, t1847: f64, t1851: f64, t598: f64, t951: f64) -> (f64, f64, f64) {
    let t2774 = t595 * t2482;
    let t2780 = t584 * t956 * t591;
    let t2782 = t2461 * t170;
    let t2787 = -0.675260332e-1_f64 * t2774 * t598 - 0.675260332e-1_f64 * t951 * t1655 - 0.571528e-1_f64 * t2780 - t1803 - t1806 + 0.285764e-1_f64 * t159 * t2782 - t1808 + t1812 + t1816 + t1825 - t1829 - t1833 - t1840 + t1844 - 0.11696447245269292414e1_f64 * t1845 + 0.17315859105681463759e2_f64 * t1847 + t1851;
    (t2774, t2782, t2787)
}
