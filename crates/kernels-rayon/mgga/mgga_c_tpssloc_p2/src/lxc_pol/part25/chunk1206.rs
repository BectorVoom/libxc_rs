//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 25 (v4rho3sigma_1) CSE chunk 1206/1226 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part25_v4rho3sigma_1_chunk1206(t81849: f64, t81852: f64, t81855: f64, t81857: f64, t81859: f64, t81861: f64, t81863: f64, t81866: f64, t81869: f64, t81874: f64, t81877: f64, t81880: f64, t81883: f64, t81887: f64, t81889: f64, t81891: f64, t81893: f64, t81895: f64, t81899: f64, t81903: f64) -> f64 {
    let t84896 = 0.2034786907144675699e0_f64 * t81849;
    let t84897 = 455.0_f64 / 648.0_f64 * t81852;
    let t84916 = -t84896 - t84897 - 0.24223653656484234512e-2_f64 * t81855 - 35.0_f64 / 96.0_f64 * t81857 + 0.84782787797694820791e-2_f64 * t81859 - 5.0_f64 / 64.0_f64 * t81861 + t81863 / 64.0_f64 + t81866 / 32.0_f64 - 0.40372756094140390853e-3_f64 * t81869 + 0.20186378047070195427e-3_f64 * t81874 + 0.10093189023535097713e-3_f64 * t81877 + t81880 / 768.0_f64 - 0.31625325607076639502e-2_f64 * t81883 - 7.0_f64 / 192.0_f64 * t81887 + 7.0_f64 / 384.0_f64 * t81889 + 5.0_f64 / 64.0_f64 * t81891 - t81893 / 256.0_f64 - t81895 / 768.0_f64 + 0.12111826828242117256e-2_f64 * t81899 + 0.60559134141210586279e-3_f64 * t81903;
    t84916
}
