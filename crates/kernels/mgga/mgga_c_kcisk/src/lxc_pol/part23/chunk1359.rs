//! MGGA_C_KCISK lxc pol — lxc_pol part 23 (v4rho3sigma_3) CSE chunk 1359/1447 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part23_v4rho3sigma_3_chunk1359<F: Float>(t1163: F, t20625: F, t6183: F, t3579: F, t5874: F, t3583: F, t3575: F, t6175: F, t1284: F, t2721: F, t18985: F, t3482: F, t20237: F, t32087: F, t33416: F, t109617: F, t109636: F, t110222: F, t110384: F, t110524: F, t110663: F, t1328: F, t32008: F, t33360: F, t33417: F, t33422: F, t33428: F, t33446: F, t3937: F, t6211: F) -> (F, F, F, F, F, F) {
    let t113861 = t6183 * t20625 * t1163;
    let t113871 = t6183 * t5874 * t3579;
    let t113875 = t6183 * t5874 * t3583;
    let t113879 = t6175 * t5874 * t3575;
    let t113888 = t2721 * t1284;
    let t113890 = t3482 * t113888 * t18985;
    let t113902 = 0.30864197530864197531e-2 * t32087 * t20237 * t33416;
    let t113904 = 0.26805555555555555556e-2 * t32008 * t113861 + 0.69444444444444444446e-2 * t110384 * t33428 - 0.69444444444444444446e-2 * t32087 * t3937 * t33422 * t3579 - 0.26805555555555555556e-2 * t32008 * t113871 + 0.13402777777777777778e-2 * t32008 * t113875 + 0.17870370370370370371e-2 * t32008 * t113879 + 0.22109259259259259258e-2 * t109617 + 0.69444444444444444446e-2 * t32087 * t3937 * t1328 * t6211 * t1163 - 0.73697530864197530861e-3 * t113890 - 0.18518518518518518519e-1 * t110524 * t33446 + 0.26805555555555555556e-2 * t110222 * t33360 + 0.26805555555555555556e-2 * t110663 * t33360 + 0.24691358024691358026e-1 * t110524 * t33417 - t113902 - 0.22109259259259259258e-2 * t109636;
    (t113861, t113871, t113875, t113879, t113890, t113904)
}
