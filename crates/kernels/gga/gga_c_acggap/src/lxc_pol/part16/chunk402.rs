//! GGA_C_ACGGAP lxc pol — lxc_pol part 16 (v4rho3sigma_8) CSE chunk 402/1223 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part16_v4rho3sigma_8_chunk402<F: Float>(t1247: F, t1815: F, t1651: F, t525: F, t1839: F, t456: F, t1844: F, t182: F, t1907: F, t119: F, t1226: F, t1228: F, t1235: F, t1246: F, t151: F, t1627: F, t1631: F, t1645: F, t1649: F) -> (F, F, F, F, F, F) {
    let t1922 = t1247 * t1815;
    let t1925 = t1651 * t525;
    let t1928 = t456 * t1839;
    let t1931 = t456 * t1844;
    let t1934 = t182 * t1907;
    let t1937 = t1226 - t1228 - F::cast_from(0.13170898365871023197e1_f64) * t1627 + F::cast_from(0.13170898365871023197e1_f64) * t1645 + t1235 + F::cast_from(0.13170898365871023197e1_f64) * t1631 - F::cast_from(0.13170898365871023197e1_f64) * t1649 - t1246 + F::cast_from(0.13170898365871023197e1_f64) * t151 * t1922 - F::cast_from(0.13170898365871023197e1_f64) * t151 * t1925 - F::cast_from(0.65854491829355115987e0_f64) * t151 * t1928 - F::cast_from(0.65854491829355115987e0_f64) * t151 * t1931 + F::cast_from(0.65854491829355115987e0_f64) * t119 * t1934;
    (t1922, t1925, t1928, t1931, t1934, t1937)
}
