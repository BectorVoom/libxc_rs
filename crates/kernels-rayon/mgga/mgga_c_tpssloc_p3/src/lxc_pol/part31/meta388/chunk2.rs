//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 31 (v4rho3sigma_7) CSE chunk 1387/2041 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1387(t17933: f64, t17958: f64, t360: f64, t1021: f64, t248: f64, t1020: f64, t10413: f64, t10891: f64, t10949: f64, t14077: f64, t14080: f64, t14136: f64, t14139: f64, t14207: f64, t1618: f64, t1622: f64, t17907: f64, t17920: f64, t17925: f64, t3048: f64, t3070: f64, t4641: f64, t4652: f64, t5857: f64, t5875: f64, t5880: f64, t5900: f64) -> (f64, f64) {
    let t17959 = t17933 + t17958;
    let t17960 = t17959 * t360;
    let t17962 = t248 * t1021 * t17960;
    let t17967 = -t14080 * t1622 / 432.0_f64 + t3048 * t5900 / 432.0_f64 - t17907 / 3456.0_f64 + t10891 * t5880 / 576.0_f64 - t3048 * t5857 / 864.0_f64 + t14207 * t1618 / 1536.0_f64 + t4641 * t4652 / 1536.0_f64 - t14077 * t1618 / 288.0_f64 + 5.0_f64 / 6912.0_f64 * t3070 * t17920 - t10413 * t17925 / 2304.0_f64 - t14136 + t14139 + t1020 * t17962 / 3072.0_f64 + t10949 * t5875 / 1536.0_f64;
    (t17959, t17967)
}
