//! MGGA_C_KCIS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 1140/1177 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part23_v4rho3sigma_5_chunk1140<F: Float>(t12930: F, t12933: F, t12940: F, t1636: F, t18352: F, t18355: F, t2268: F, t27702: F, t28658: F, t28666: F, t4480: F, t4500: F, t8240: F, t8251: F, t97961: F, t97976: F, t97977: F, t97979: F, t97984: F, t97989: F, t97990: F, t97993: F, t97996: F, t98956: F) -> (F,) {
    let t98957 = -12.0 * t12940 * t1636 * t28658 - 6.0 * t12940 * t4500 * t8240 + 2.0 * t18352 * t2268 * t4480 + 2.0 * t4480 * t4500 * t8251 - t12930 * t8251 + 4.0 * t12933 * t28658 + 4.0 * t12933 * t28666 + 2.0 * t18355 * t27702 + t97961 + t97976 + t97977 + t97979 + t97984 - t97989 + t97990 - t97993 - t97996 - t98956;
    (t98957,)
}
