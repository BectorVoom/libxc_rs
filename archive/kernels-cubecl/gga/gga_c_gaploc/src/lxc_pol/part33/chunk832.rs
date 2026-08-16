//! GGA_C_GAPLOC lxc pol — lxc_pol part 33 (v4rho2sigma2_16) CSE chunk 832/1464 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part33_v4rho2sigma2_16_chunk832<F: Float>(t2822: F, t448: F, t1306: F, t999: F, t158: F, t2754: F, t123: F, t488: F, t1063: F, t1358: F, t2268: F, t2762: F, t2780: F, t2784: F, t3808: F, t3818: F, t3833: F, t4141: F, t6499: F, t6501: F, t6505: F, t6527: F, t6534: F, t7997: F, t8001: F, t8005: F, t8013: F, t8016: F) -> (F, F, F) {
    let t8019 = t2822 * t448;
    let t8022 = t999 * t1306;
    let t8025 = t158 * t2754;
    let t8026 = t8025 * t123;
    let t8027 = t8026 * t488;
    let t8038 = F::cast_from(0.63233348079280332442e-2_f64) * t6499 - F::cast_from(0.63233348079280332442e-2_f64) * t6501 - F::cast_from(0.47425011059460249332e-2_f64) * t6505 + F::cast_from(0.1138200265427045984e0_f64) * t2268 * t7997 + F::cast_from(0.56910013271352299198e-1_f64) * t1063 * t8001 + F::cast_from(0.28455006635676149599e-1_f64) * t1063 * t8005 + F::cast_from(0.56910013271352299198e-1_f64) * t3833 * t2780 + F::cast_from(0.7588001769513639893e-1_f64) * t3818 * t2780 + F::cast_from(0.56910013271352299198e-1_f64) * t2268 * t8013 - F::cast_from(0.85365019907028448797e-1_f64) * t2268 * t8016 - F::cast_from(0.56910013271352299198e-1_f64) * t1063 * t8019 - F::cast_from(0.28455006635676149599e-1_f64) * t1063 * t8022 - F::cast_from(0.63233348079280332442e-2_f64) * t1358 * t8027 + F::cast_from(0.31616674039640166222e-2_f64) * t4141 * t2762 - F::cast_from(0.63233348079280332442e-2_f64) * t3808 * t2762 - F::cast_from(0.31616674039640166222e-2_f64) * t4141 * t2784 - F::cast_from(0.47425011059460249332e-2_f64) * t6527 + F::cast_from(0.47425011059460249332e-2_f64) * t6534;
    (t8025, t8026, t8038)
}
