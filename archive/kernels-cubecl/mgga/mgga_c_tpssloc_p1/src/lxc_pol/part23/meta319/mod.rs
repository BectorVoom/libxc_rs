//! MGGA_C_TPSSLOC lxc pol kernel — _part23_v4rho4_4 meta319 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1079;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_meta319<F: Float>(t22132: F, t974: F, t11759: F, t20234: F, t21745: F, t3440: F, t11649: F, t1174: F, t1726: F, t18310: F, t18312: F, t18314: F, t18321: F, t18325: F, t18327: F, t18330: F, t18333: F, t22012: F, t22015: F, t22116: F, t22119: F, t22129: F, t488: F, t4889: F, t6178: F, t6184: F, t6188: F) -> (F, F, F, F, F) {
        let (t22133, t22136, t22137, t22149, t22152) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1079::<F>(t22132, t974, t11759, t20234, t21745, t3440, t11649, t1174, t1726, t18310, t18312, t18314, t18321, t18325, t18327, t18330, t18333, t22012, t22015, t22116, t22119, t22129, t488, t4889, t6178, t6184, t6188);
    (t22133, t22136, t22137, t22149, t22152)
}
