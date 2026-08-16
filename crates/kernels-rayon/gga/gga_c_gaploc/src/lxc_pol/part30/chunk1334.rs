//! GGA_C_GAPLOC lxc pol — lxc_pol part 30 (v4rho2sigma2_13) CSE chunk 1334/1436 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part30_v4rho2sigma2_13_chunk1334(t3040: f64, t7596: f64, t7590: f64, t16251: f64, t2103: f64, t3447: f64, t10713: f64, t4673: f64, t10717: f64, t11013: f64, t5771: f64, t10783: f64, t10948: f64, t1457: f64, t32371: f64, t33851: f64, t33853: f64, t33857: f64, t33859: f64, t33861: f64, t33863: f64, t33865: f64, t7653: f64) -> f64 {
    let t33867 = 0.71500979903700853338e0_f64 * t7596 * t3040;
    let t33869 = 0.35750489951850426669e0_f64 * t7590 * t3040;
    let t33872 = 0.15889106645266856297e0_f64 * t2103 * t16251 * t3447;
    let t33878 = 0.95334639871601137784e0_f64 * t2103 * t4673 * t10713;
    let t33881 = 0.95334639871601137784e0_f64 * t2103 * t4673 * t10717;
    let t33883 = 0.95334639871601137784e0_f64 * t5771 * t11013;
    let t33887 = t33851 - t33853 - 0.14300195980740170668e1_f64 * t10948 * t7653 - t33857 - t33859 - t33861 + t33863 + t33865 + t33867 + t33869 - t33872 + 0.95334639871601137784e0_f64 * t2103 * t4673 * t10783 + t33878 + t33881 + t33883 + 0.71500979903700853338e0_f64 * t2103 * t1457 * t32371;
    t33887
}
