//! MGGA_C_REVTPSS kxc pol — kxc_pol part 4 (v3rho3_1) CSE chunk 1393/1428 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_kxc_pol_part4_v3rho3_1_chunk1393<F: Float>(t12784: F, t12866: F, t12910: F, t17619: F, t17622: F, t17625: F, t17629: F, t17635: F, t17640: F, t17646: F, t17651: F, t17654: F, t17658: F, t17662: F, t3625: F, t5402: F) -> F {
    let t17665 = -t17619 - t17622 + F::cast_from(0.42874018118069736972e-3_f64) * t12910 * t17625 + t17629 / F::cast_from(1296.0_f64) - F::cast_from(0.28582678745379824648e-3_f64) * t12784 * t5402 - F::cast_from(0.28582678745379824648e-3_f64) * t3625 * t17635 - F::cast_from(0.14291339372689912324e-3_f64) * t3625 * t17640 - F::cast_from(0.28582678745379824648e-3_f64) * t3625 * t17646 + F::cast_from(0.28582678745379824648e-3_f64) * t12866 * t17651 - F::cast_from(0.57165357490759649296e-3_f64) * t17654 * t17658 + F::cast_from(0.28582678745379824648e-3_f64) * t12866 * t17662;
    t17665
}
