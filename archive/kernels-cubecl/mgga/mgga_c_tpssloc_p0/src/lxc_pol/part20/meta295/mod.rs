//! MGGA_C_TPSSLOC lxc pol kernel — _part20_v4rho4_1 meta295 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1509;
use chunk1::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1510;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_meta295<F: Float>(t10993: F, t973: F, t2770: F, t976: F, t9288: F, t974: F, t248: F, t3101: F, t3132: F, t3130: F, t1025: F, t1041: F, t1046: F, t10932: F, t10937: F, t10944: F, t10949: F, t10952: F, t10957: F, t10962: F, t10965: F, t10972: F, t10982: F, t10985: F, t10988: F, t2960: F, t3043: F, t3048: F, t3057: F, t3064: F, t3073: F, t3117: F, t3134: F, t3143: F, t3148: F, t3153: F, t10431: F, t10513: F, t10929: F, t349: F, t225: F, t3167: F, t3166: F, t990: F, t10358: F, t381: F, t1049: F, t3020: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t10994, t10997, t11002, t11003, t11005) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1509::<F>(t10993, t973, t2770, t976, t9288, t974, t248, t3101, t3132, t3130, t1025, t1041, t1046, t10932, t10937, t10944, t10949, t10952, t10957, t10962, t10965, t10972, t10982, t10985, t10988, t2960, t3043, t3048, t3057, t3064, t3073, t3117, t3134, t3143, t3148, t3153);
        let (t11007, t11008, t11010, t11013, t11016, t11018) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1510::<F>(t10431, t10513, t10929, t11005, t349, t225, t3167, t3166, t990, t10358, t381, t1049, t3020);
    (t10994, t10997, t11002, t11003, t11007, t11008, t11010, t11013, t11016, t11018)
}
