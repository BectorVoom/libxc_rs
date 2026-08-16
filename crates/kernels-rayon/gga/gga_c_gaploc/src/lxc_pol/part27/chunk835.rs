//! GGA_C_GAPLOC lxc pol — lxc_pol part 27 (v4rho2sigma2_10) CSE chunk 835/1468 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part27_v4rho2sigma2_10_chunk835(t2822: f64, t448: f64, t1306: f64, t999: f64, t158: f64, t2754: f64, t123: f64, t488: f64, t1063: f64, t1358: f64, t2268: f64, t2762: f64, t2780: f64, t2784: f64, t3808: f64, t3818: f64, t3833: f64, t4141: f64, t6499: f64, t6501: f64, t6505: f64, t6527: f64, t6534: f64, t7997: f64, t8001: f64, t8005: f64, t8013: f64, t8016: f64) -> (f64, f64, f64) {
    let t8019 = t2822 * t448;
    let t8022 = t999 * t1306;
    let t8025 = t158 * t2754;
    let t8026 = t8025 * t123;
    let t8027 = t8026 * t488;
    let t8038 = 0.63233348079280332442e-2_f64 * t6499 - 0.63233348079280332442e-2_f64 * t6501 - 0.47425011059460249332e-2_f64 * t6505 + 0.1138200265427045984e0_f64 * t2268 * t7997 + 0.56910013271352299198e-1_f64 * t1063 * t8001 + 0.28455006635676149599e-1_f64 * t1063 * t8005 + 0.56910013271352299198e-1_f64 * t3833 * t2780 + 0.7588001769513639893e-1_f64 * t3818 * t2780 + 0.56910013271352299198e-1_f64 * t2268 * t8013 - 0.85365019907028448797e-1_f64 * t2268 * t8016 - 0.56910013271352299198e-1_f64 * t1063 * t8019 - 0.28455006635676149599e-1_f64 * t1063 * t8022 - 0.63233348079280332442e-2_f64 * t1358 * t8027 + 0.31616674039640166222e-2_f64 * t4141 * t2762 - 0.63233348079280332442e-2_f64 * t3808 * t2762 - 0.31616674039640166222e-2_f64 * t4141 * t2784 - 0.47425011059460249332e-2_f64 * t6527 + 0.47425011059460249332e-2_f64 * t6534;
    (t8025, t8026, t8038)
}
