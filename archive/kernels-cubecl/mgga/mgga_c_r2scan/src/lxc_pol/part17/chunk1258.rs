//! MGGA_C_R2SCAN lxc pol — lxc_pol part 17 (v4rho3sigma_7) CSE chunk 1258/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part17_v4rho3sigma_7_chunk1258<F: Float>(t322: F, t44642: F, t1127: F, t1129: F, t1131: F, t1133: F, t1135: F, t2958: F, t335: F, t337: F, t339: F, t341: F, t343: F, t3522: F, t9707: F, t9709: F, t9715: F) -> (F, F) {
    let t332 = F::cast_from(0.25e1_f64) < t322;
    let t44685 = piecewise3::<F>(t332, F::cast_from(0.0_f64), t44642);
    let t44715 = -F::cast_from(0.64e0_f64) * t44685 - F::cast_from(0.4607056813647e1_f64) * t335 * t44685 + F::cast_from(0.122462410087e2_f64) * t337 * t44685 - F::cast_from(0.957855118103e1_f64) * t339 * t44685 + F::cast_from(0.3101306810232e1_f64) * t341 * t44685 - F::cast_from(0.362942158544e0_f64) * t343 * t44685 - F::cast_from(0.9214113627294e1_f64) * t2958 * t3522 - F::cast_from(0.8704e0_f64) * t9707 * t1127 + F::cast_from(0.1469548921044e3_f64) * t1129 * t9709 - F::cast_from(0.22988522834472e3_f64) * t1129 * t9715 - F::cast_from(0.22988522834472e3_f64) * t1131 * t9709 + F::cast_from(0.18607840861392e3_f64) * t1131 * t9715 + F::cast_from(0.12405227240928e3_f64) * t1133 * t9709 - F::cast_from(0.4355305902528e2_f64) * t1133 * t9715 - F::cast_from(0.2177652951264e2_f64) * t1135 * t9709;
    (t44685, t44715)
}
