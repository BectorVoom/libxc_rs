//! MGGA_C_R2SCAN lxc pol — lxc_pol part 8 (v4rho4_3) CSE chunk 1409/1467 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part8_v4rho4_3_chunk1409<F: Float>(t51: F, t101: F, t1217: F, t2517: F, t2520: F, t3008: F, t32178: F, t32181: F, t32189: F, t419: F, t8584: F, t906: F, t9870: F, t9875: F, t20925: F, t25754: F, t25756: F, t30123: F, t30132: F, t30134: F, t30136: F, t30138: F, t30142: F, t30146: F, t3224: F, t34141: F, t34144: F, t34162: F, t549: F, t551: F, t552: F, t562: F, t568: F, t7250: F, zeta_threshold: F) -> (F,) {
    let t52 = t51 <= zeta_threshold;
    let t34178 = piecewise3(t52, 0.0, 40.0 / 81.0 * t9870 * t419 + 20.0 / 9.0 * t3008 * t1217 - 10.0 / 9.0 * t2517 * t32178 - 20.0 / 3.0 * t2520 * t32181 + 10.0 / 3.0 * t906 * t8584 + 10.0 / 9.0 * t9875 * t419 + 5.0 / 3.0 * t101 * t32189);
    let t34187 = 0.10401866088065122276e1 * t30123 - t25754 - 0.57131963037208741164e-1 * t25756 - t20925 - 0.16463622957338778996e-1 * t30132 - 0.35126785941778018867e0 * t30134 + 0.17563392970889009434e0 * t30136 - 0.34930954652346593433e-1 * t30138 + 0.17465477326173296717e-1 * t30142 + 0.34672886960217074253e0 * t30146 - 0.43341108700271342816e-1 * t34141 * t562 - 0.13002332610081402845e0 * t34144 * t568 - 0.43341108700271342816e-1 * t549 * t551 * t552 * (t34162 / 2.0 + t34178 / 2.0) - 0.39006997830244208535e0 * t7250 * t3224;
    (t34187,)
}
