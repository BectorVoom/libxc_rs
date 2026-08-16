//! MGGA_C_TPSSLOC lxc pol kernel — _part20_v4rho4_1 meta656 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2424;
use chunk1::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2425;
use chunk2::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2426;
use chunk3::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2427;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_meta656(t49222: f64, t942: f64, t951: f64, t959: f64, t10524: f64, t1580: f64, t42110: f64, t42113: f64, t10723: f64, t13658: f64, t10526: f64, t10623: f64, t13659: f64, t13732: f64, t2940: f64, t4483: f64, t4489: f64, t49278: f64, t49280: f64, t49282: f64, t49426: f64, t49485: f64, t49488: f64, t49491: f64, t48760: f64, t49256: f64, t49259: f64, t49262: f64, t49268: f64, t49271: f64, t49273: f64, t49276: f64, t49530: f64, t49563: f64, t3041: f64, t607: f64, t1023: f64, t3120: f64, t10390: f64, t14501: f64, t10422: f64, t13761: f64, t3070: f64, t1020: f64, t1021: f64, t1031: f64, t10413: f64, t13941: f64, t14093: f64, t1539: f64, t248: f64, t3071: f64, t3088: f64, t3117: f64, t360: f64, t378: f64, t42514: f64, t42518: f64, t4342: f64, t4347: f64, t4616: f64, t48670: f64, t48674: f64, t1615: f64, t3040: f64, t10403: f64, t14214: f64, t3030: f64, t4552: f64, t3032: f64, t3129: f64, t1022: f64, t10408: f64, t10937: f64, t14174: f64, t14207: f64, t14211: f64, t14212: f64, t14220: f64, t14222: f64, t14235: f64, t14491: f64, t2244: f64, t2250: f64, t2770: f64, t3114: f64, t3123: f64, t3134: f64, t42483: f64, t42508: f64, t42530: f64, t4337: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t49567, t49572, t49575, t49585) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2424(t49222, t942, t951, t959, t10524, t1580, t42110, t42113, t10723, t13658, t10526, t10623, t13659, t13732, t2940, t4483, t4489, t49278, t49280, t49282, t49426, t49485, t49488, t49491);
        let t49588 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2425(t48760, t49256, t49259, t49262, t49268, t49271, t49273, t49276, t49530, t49563, t49567, t49572, t49575, t49585);
        let (t49594, t49599, t49609) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2426(t3041, t607, t1023, t3120, t10390, t14501, t10422, t13761, t3070, t1020, t1021, t1031, t10413, t13941, t14093, t1539, t248, t3071, t3088, t3117, t360, t378, t42514, t42518, t4342, t4347, t4616, t48670, t48674, t49588);
        let (t49616, t49649, t49650, t49654) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2427(t1615, t3120, t3040, t10403, t10422, t14214, t3030, t4552, t3032, t3129, t1022, t10408, t10413, t10937, t14174, t14207, t14211, t14212, t14220, t14222, t14235, t14491, t2244, t2250, t2770, t3071, t3114, t3117, t3123, t3134, t42483, t42508, t42530, t4337, t49594);
    (t49567, t49572, t49575, t49585, t49588, t49599, t49609, t49616, t49649, t49650, t49654)
}
