//! GGA_C_GAPLOC lxc pol — lxc_pol part 21 (v4rho2sigma2_4) CSE chunk 1286/1466 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part21_v4rho2sigma2_4_chunk1286(t11013: f64, t5771: f64, t10972: f64, t4614: f64, t813: f64, t29001: f64, t14626: f64, t3483: f64, t3447: f64, t833: f64, t2718: f64, t8556: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t33883 = 0.95334639871601137784e0_f64 * t5771 * t11013;
    let t33891 = 0.12269736305254639897e2_f64 * t813 * t4614 * t10972;
    let t33892 = 0.63904876589867916128e-1_f64 * t29001;
    let t33901 = 0.20449560508757733161e1_f64 * t813 * t14626 * t3483;
    let t33905 = 0.51123901271894332903e1_f64 * t833 * t14626 * t3447;
    let t33907 = 0.47667319935800568892e0_f64 * t2718 * t8556;
    (t33883, t33891, t33892, t33901, t33905, t33907)
}
