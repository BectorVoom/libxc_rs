//! GGA_C_GAPLOC lxc pol — lxc_pol part 18 (v4rho2sigma2_1) CSE chunk 1252/1268 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part18_v4rho2sigma2_1_chunk1252<F: Float>(t1445: F, t31866: F, t597: F, t10123: F, t10151: F, t10480: F, t1305: F, t1429: F, t1450: F, t1456: F, t1457: F, t188: F, t189: F, t193: F, t31291: F, t31502: F, t31509: F, t31623: F, t31655: F, t34983: F, t34986: F, t34991: F, t34994: F, t34996: F, t34999: F, t35021: F, t4614: F, t4679: F, t549: F, t567: F) -> (F,) {
    let t35024 = 0.11502877786176224903e2 * t597 * t1445 * t31866;
    let t35025 = t34983 - t34986 + 0.79445533226334281486e-1 * t1429 * t549 * t31655 + t34991 + t34994 - t34996 - t34999 + 0.61348681526273199482e1 * t567 * t4614 * t10123 - t31291 + 0.35750489951850426669e0 * t188 * t189 * t31623 * t193 + 0.71500979903700853338e0 * t1456 * t1457 * t31502 + 0.71500979903700853338e0 * t4679 * t10480 - 0.23005755572352449806e1 * t1450 * t1445 * t10151 * t1305 + 0.23005755572352449806e1 * t567 * t1445 * t31509 + t35021 + t35024;
    (t35025,)
}
