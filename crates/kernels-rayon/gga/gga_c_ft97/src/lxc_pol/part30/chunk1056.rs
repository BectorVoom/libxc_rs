//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 1056/1184 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk1056(t1403: f64, t27825: f64, t2336: f64, t35760: f64, t1091: f64, t141441: f64, t141447: f64, t141461: f64, t141468: f64, t1425: f64, t1526: f64, t193: f64, t2320: f64, t24181: f64, t27469: f64, t27860: f64, t27863: f64, t27866: f64, t27869: f64, t33540: f64, t33547: f64, t33552: f64, t35757: f64, t35761: f64, t35766: f64, t3704: f64, t3746: f64, t3837: f64, t5996: f64, t6002: f64, t6062: f64, t666: f64, t6745: f64, t6838: f64, t684: f64) -> f64 {
    let t151123 = t1403 * t27825;
    let t151126 = t1403 * t2336 * t35760;
    let t151139 = t141441 / 18.0_f64 - t1403 * t193 * t24181 * t3837 - t1403 * t3704 * t1425 * t3746 / 9.0_f64 - t141447 - t141461 / 36.0_f64 - t6002 * t27863 / 9.0_f64 - t6002 * t27866 / 9.0_f64 + t6002 * t27869 / 27.0_f64 - t6002 * t27860 / 9.0_f64 + t1403 * t666 * t6062 * t1091 / 18.0_f64 + t5996 * t35761 / 18.0_f64 + t6745 * t33547 / 18.0_f64 + t141468 / 18.0_f64 - t151123 / 9.0_f64 - t151126 / 54.0_f64 + t1403 * t666 * t6838 * t684 / 18.0_f64 - t33540 * t35766 / 6.0_f64 - t35757 * t33552 / 6.0_f64 - t1526 * t2320 * t27469 / 12.0_f64;
    t151139
}
