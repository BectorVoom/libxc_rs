//! MGGA_C_KCIS kxc pol — kxc_pol part 5 (v3rho3_2) CSE chunk 617/1260 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_kxc_pol_part5_v3rho3_2_chunk617<F: Float>(t233: F, t4535: F, t1295: F, t1658: F, t1877: F, t911: F, t1876: F, t915: F, t1071: F, t359: F, t1646: F, t829: F) -> (F, F, F, F, F, F) {
    let t4536 = t233 * t4535;
    let t4538 = t1658 * t1295;
    let t4539 = t233 * t4538;
    let t4541 = t911 * t1877;
    let t4543 = t915 * t1876;
    let t4544 = t233 * t4543;
    let t4546 = t359 * t1071;
    let t4547 = t1646 * t829;
    (t4536, t4539, t4541, t4544, t4546, t4547)
}
