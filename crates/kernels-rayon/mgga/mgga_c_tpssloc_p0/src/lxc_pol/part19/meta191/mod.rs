//! MGGA_C_TPSSLOC lxc pol kernel — _part19_v4rho4_0 meta191 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk850;
use chunk1::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk851;
use chunk2::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk852;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_meta191(t10103: f64, t858: f64, t856: f64, t68: f64, t2719: f64, t865: f64, t2742: f64, t2718: f64, t10047: f64, t10049: f64, t259: f64, t2597: f64, t2713: f64, t2720: f64, t2743: f64, t855: f64, t866: f64, t9520: f64, t9585: f64, t9587: f64, t9590: f64, t9593: f64, t193: f64, t202: f64, t2379: f64, t2522: f64, t2523: f64, t2553: f64, t262: f64, t4314: f64, t766: f64, t776: f64, t870: f64, t9450: f64, t9457: f64, t9458: f64, t9463: f64, t9469: f64, t9470: f64, t9476: f64, t9484: f64, t9496: f64, t9516: f64, t2745: f64, t2752: f64, t1877: f64, t868: f64, t9684: f64, t9715: f64, t9718: f64, t9724: f64, t9727: f64, t9780: f64, t9789: f64, t9863: f64, t9865: f64, t9867: f64, t9870: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t10104, t10108, t10110, t10112, t10116, t10121) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk850(t10103, t858, t856, t68, t2719, t865, t2742, t2718, t10047, t10049, t259, t2597, t2713, t2720, t2743, t855, t866, t9520, t9585, t9587, t9590, t9593);
        let t10125 = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk851(t10121, t193, t202, t2379, t2522, t2523, t2553, t262, t4314, t766, t776, t870, t9450, t9457, t9458, t9463, t9469, t9470, t9476, t9484, t9496, t9516);
        let (t10126, t10134, t10138) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk852(t2745, t870, t2553, t262, t2752, t1877, t2522, t4314, t776, t868, t9684, t9715, t9718, t9724, t9727, t9780, t9789, t9863, t9865, t9867, t9870);
    (t10104, t10108, t10110, t10112, t10116, t10121, t10125, t10126, t10134, t10138)
}
