//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 984/1189 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk984<F: Float>(t637: F, t7368: F, t1359: F, t7242: F, t32905: F, t2253: F, t5889: F, t32901: F, t1570: F, t7312: F, t32063: F, t32913: F, t7366: F) -> (F, F, F, F, F, F, F) {
    let t139213 = t637 * t7368;
    let t139214 = t7242 * t1359;
    let t139224 = t637 * t32905;
    let t139240 = t5889 * t2253;
    let t139241 = t139240 * t32901;
    let t139248 = t7312 * t1570;
    let t139254 = t7366 * t32063 * t32913;
    (t139213, t139214, t139224, t139240, t139241, t139248, t139254)
}
