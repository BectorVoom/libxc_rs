//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta448 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1804;
use chunk1::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1805;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta448(t1362: f64, t19815: f64, t3799: f64, t6417: f64, t6422: f64, t1307: f64, t6330: f64, t12351: f64, t820: f64, t1799: f64, t5187: f64, t3870: f64, t1367: f64, t19631: f64, t16336: f64, t1831: f64, t12308: f64, t12325: f64, t12330: f64, t12335: f64, t1363: f64, t1369: f64, t16321: f64, t16346: f64, t16350: f64, t16354: f64, t3778: f64, t3783: f64, t5240: f64, t5310: f64, t5314: f64, t6427: f64, t6431: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t19904, t19915, t19917, t19919, t19921, t19924, t19926) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1804(t1362, t19815, t3799, t6417, t6422, t1307, t6330, t12351, t820, t1799, t5187, t3870);
        let (t19930, t19933, t19939) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1805(t1367, t19631, t820, t16336, t1831, t12308, t12325, t12330, t12335, t1363, t1369, t16321, t16346, t16350, t16354, t19904, t19915, t19917, t19921, t19926, t3778, t3783, t5240, t5310, t5314, t6422, t6427, t6431);
    (t19904, t19915, t19917, t19919, t19921, t19924, t19926, t19930, t19933, t19939)
}
