//! MGGA_C_TPSSLOC lxc pol kernel — _part19_v4rho4_0 meta355 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1284;
use chunk1::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1285;
use chunk2::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1286;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_meta355<F: Float>(t41654: F, t242: F, t281: F, t283: F, t136: F, t2826: F, t41705: F, t10304: F, t41693: F, t41715: F, t908: F, t41644: F, t41937: F, t41940: F, t41943: F, t41945: F, t41948: F, t41951: F, t41954: F, t41957: F, t41855: F, t41878: F, t41931: F, t2853: F, t2860: F, t10770: F, t919: F, t2862: F, t10655: F, t10737: F, t10632: F, t10753: F, t10757: F, t10772: F, t10805: F, t10806: F, t10811: F, t10813: F, t10820: F, t2861: F, t2863: F, t2880: F, t2886: F, t2888: F, t2900: F, t2907: F, t2924: F, t2925: F, t2930: F, t2933: F, t41804: F, t41813: F, t41816: F, t41821: F, t41826: F, t41827: F, t931: F, t943: F, t951: F) -> (F, F, F, F, F, F, F, F, F) {
        let (t41959, t41961, t41962, t41964, t41967, t41970, t41973) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1284::<F>(t41654, t242, t281, t283, t136, t2826, t41705, t10304, t41693, t41715, t908, t41644);
        let t41975 = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1285::<F>(t41937, t41940, t41943, t41945, t41948, t41951, t41954, t41957, t41959, t41962, t41964, t41967, t41970, t41973);
        let (t41977, t41987, t41992, t41993) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1286::<F>(t41855, t41878, t41931, t41975, t2853, t2860, t10770, t919, t2862, t10655, t10737, t10632, t10753, t10757, t10772, t10805, t10806, t10811, t10813, t10820, t2861, t2863, t2880, t2886, t2888, t2900, t2907, t2924, t2925, t2930, t2933, t41804, t41813, t41816, t41821, t41826, t41827, t931, t943, t951);
    (t41961, t41964, t41967, t41970, t41973, t41977, t41987, t41992, t41993)
}
