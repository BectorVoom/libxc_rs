//! MGGA_C_TPSSLOC lxc pol kernel — _part19_v4rho4_0 meta355 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1284;
use chunk1::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1285;
use chunk2::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1286;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_meta355(t41654: f64, t242: f64, t281: f64, t283: f64, t136: f64, t2826: f64, t41705: f64, t10304: f64, t41693: f64, t41715: f64, t908: f64, t41644: f64, t41937: f64, t41940: f64, t41943: f64, t41945: f64, t41948: f64, t41951: f64, t41954: f64, t41957: f64, t41855: f64, t41878: f64, t41931: f64, t2853: f64, t2860: f64, t10770: f64, t919: f64, t2862: f64, t10655: f64, t10737: f64, t10632: f64, t10753: f64, t10757: f64, t10772: f64, t10805: f64, t10806: f64, t10811: f64, t10813: f64, t10820: f64, t2861: f64, t2863: f64, t2880: f64, t2886: f64, t2888: f64, t2900: f64, t2907: f64, t2924: f64, t2925: f64, t2930: f64, t2933: f64, t41804: f64, t41813: f64, t41816: f64, t41821: f64, t41826: f64, t41827: f64, t931: f64, t943: f64, t951: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t41959, t41961, t41962, t41964, t41967, t41970, t41973) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1284(t41654, t242, t281, t283, t136, t2826, t41705, t10304, t41693, t41715, t908, t41644);
        let t41975 = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1285(t41937, t41940, t41943, t41945, t41948, t41951, t41954, t41957, t41959, t41962, t41964, t41967, t41970, t41973);
        let (t41977, t41987, t41992, t41993) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1286(t41855, t41878, t41931, t41975, t2853, t2860, t10770, t919, t2862, t10655, t10737, t10632, t10753, t10757, t10772, t10805, t10806, t10811, t10813, t10820, t2861, t2863, t2880, t2886, t2888, t2900, t2907, t2924, t2925, t2930, t2933, t41804, t41813, t41816, t41821, t41826, t41827, t931, t943, t951);
    (t41961, t41964, t41967, t41970, t41973, t41977, t41987, t41992, t41993)
}
