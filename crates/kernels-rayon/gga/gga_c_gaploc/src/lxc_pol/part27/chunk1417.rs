//! GGA_C_GAPLOC lxc pol — lxc_pol part 27 (v4rho2sigma2_10) CSE chunk 1417/1468 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part27_v4rho2sigma2_10_chunk1417(t12210: f64, t2009: f64, t2066: f64, t32778: f64, t32785: f64, t32791: f64, t32806: f64, t32813: f64, t32815: f64, t32818: f64, t32821: f64, t32824: f64, t32827: f64, t32829: f64, t32832: f64, t32835: f64, t32839: f64, t3732: f64, t5724: f64) -> f64 {
    let t38930 = -t32778 - t32785 - t32791 - t32806 + t32813 + t32815 - t32818 - t32821 + t32824 + t32827 - 0.35750489951850426669e0_f64 * t12210 * t5724 - 0.71500979903700853338e0_f64 * t2066 * t3732 * t2009 - t32829 - t32832 + t32835 - t32839;
    t38930
}
