//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1847/1850 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1847<F: Float>(t118: F, t1312: F, t1502: F, t1518: F, t18245: F, t1843: F, t1847: F, t1911: F, t22633: F, t22747: F, t22758: F, t23094: F, t25043: F, t25045: F, t30138: F, t4248: F, t508: F, t511: F, t569: F, t5877: F, t5920: F, t5921: F, t651: F, t6765: F, t6773: F, t6934: F, t75941: F, t7889: F, t87051: F, t87064: F, t87227: F, t87237: F, t89771: F, t91789: F, t92446: F, t92453: F, t92465: F, t92466: F, t92469: F, t92490: F, t92500: F, t92504: F, t93: F, t94: F) -> F {
    let t92516 = (F::new(2.0) * t1312 * t87051 + F::new(8.0) * t1518 * t75941 + F::new(12.0) * t18245 * t5920 + F::new(8.0) * t22633 * t4248 + F::new(8.0) * t22633 * t7889 + F::new(24.0) * t30138 * t5920 + F::new(6.0) * t87237 * t93 + F::new(12.0) * t87064 + t87227) * t569 - F::new(6.0) * t94 * t87237 * t508 - F::new(4.0) * t1502 * t25043 - F::new(4.0) * t22747 * t1843 - F::new(6.0) * t5877 * t6765 + F::new(6.0) * t6773 * t6934 - t87227 * t508 + F::new(4.0) * t22758 * t1911 - t118 * (t89771 + t91789) + F::new(4.0) * t1847 * t23094 + t511 * (t92446 + t92453 + t92465 + t92466 + t92469 + t92490 + t92500 + t92504) - F::new(8.0) * t651 * t25043 * t1518 - F::new(24.0) * t4248 * t25045 - F::new(12.0) * t18245 * t5921;
    t92516
}
