//! MGGA_C_R2SCAN lxc pol — lxc_pol part 18 (v4rho3sigma_8) CSE chunk 703/1264 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part18_v4rho3sigma_8_chunk703<F: Float>(t1691: F, t713: F, t5270: F, t721: F, t1923: F, t1945: F, t1957: F, t1981: F, t2017: F, t202: F, t208: F, t390: F, t5444: F, t5451: F, t5513: F, t5524: F, t5527: F, t5530: F, t5531: F, t5534: F, t5537: F, t5539: F, t5543: F, t5549: F, t674: F, t705: F, t718: F) -> F {
    let t5556 = t713 * t1691;
    let t5559 = t721 * t5270;
    let t5562 = -F::new(0.30822e0) * t390 * t5513 + F::new(1.0) * t202 * t5524 - F::new(0.31168546390226634765e3) * t1945 * t5527 - F::new(0.12304822629859687989e5) * t5530 * t5531 - F::new(0.35089341735807877242e1) * t705 * t5534 + t5444 + F::new(0.91082604192152556044e5) * t5537 * t5539 + F::new(0.51947577317044391277e2) * t718 * t5543 - F::new(2.0) * t674 * t208 * t5549 - F::new(0.57895126195293126242e3) * t1957 * t2017 * t1923 + F::new(0.10526802520742363173e2) * t718 * t5556 + F::new(0.6233709278045326953e3) * t1981 * t5559 + t5451;
    t5562
}
