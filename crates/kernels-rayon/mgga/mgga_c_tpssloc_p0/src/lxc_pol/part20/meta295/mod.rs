//! MGGA_C_TPSSLOC lxc pol kernel — _part20_v4rho4_1 meta295 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1509;
use chunk1::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1510;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_meta295(t10993: f64, t973: f64, t2770: f64, t976: f64, t9288: f64, t974: f64, t248: f64, t3101: f64, t3132: f64, t3130: f64, t1025: f64, t1041: f64, t1046: f64, t10932: f64, t10937: f64, t10944: f64, t10949: f64, t10952: f64, t10957: f64, t10962: f64, t10965: f64, t10972: f64, t10982: f64, t10985: f64, t10988: f64, t2960: f64, t3043: f64, t3048: f64, t3057: f64, t3064: f64, t3073: f64, t3117: f64, t3134: f64, t3143: f64, t3148: f64, t3153: f64, t10431: f64, t10513: f64, t10929: f64, t349: f64, t225: f64, t3167: f64, t3166: f64, t990: f64, t10358: f64, t381: f64, t1049: f64, t3020: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t10994, t10997, t11002, t11003, t11005) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1509(t10993, t973, t2770, t976, t9288, t974, t248, t3101, t3132, t3130, t1025, t1041, t1046, t10932, t10937, t10944, t10949, t10952, t10957, t10962, t10965, t10972, t10982, t10985, t10988, t2960, t3043, t3048, t3057, t3064, t3073, t3117, t3134, t3143, t3148, t3153);
        let (t11007, t11008, t11010, t11013, t11016, t11018) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1510(t10431, t10513, t10929, t11005, t349, t225, t3167, t3166, t990, t10358, t381, t1049, t3020);
    (t10994, t10997, t11002, t11003, t11007, t11008, t11010, t11013, t11016, t11018)
}
