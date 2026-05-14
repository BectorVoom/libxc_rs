//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 962/1171 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk962<F: Float>(t27879: F, t303: F, t4773: F, t7726: F, t26715: F, t26718: F, t27808: F, t27838: F, t27843: F, t27847: F, t27849: F, t27852: F, t27857: F, t27860: F, t27865: F, t27868: F, t27871: F, t27874: F, t27877: F, t7687: F, t7696: F, t7703: F, t8034: F, t8042: F) -> (F, F, F, F) {
    let t27880 = t303 * t27879;
    let t27882 = t7726 * t4773;
    let t27883 = t303 * t27882;
    let t27889 = 0.16581944444444444444e-2 * t27838 - 0.13901041666666666667e-2 * t7703 * t27808 + 0.16581944444444444444e-2 * t27843 + 0.33163888888888888888e-2 * t27847 + 0.11054629629629629629e-2 * t27849 - 0.44218518518518518517e-2 * t27852 - 0.18534722222222222222e-2 * t7696 * t8034 + 0.23168402777777777778e-3 * t27857 + 0.16581944444444444444e-2 * t27860 + 0.69505208333333333333e-3 * t7687 * t8034 - 0.16581944444444444444e-2 * t27865 + 0.11054629629629629629e-2 * t27868 - 0.33163888888888888888e-2 * t27871 + 0.27636574074074074073e-2 * t27874 - 0.16581944444444444444e-2 * t27877 + 0.24872916666666666666e-2 * t27880 - 0.24872916666666666666e-2 * t27883 - 0.18534722222222222222e-2 * t7696 * t8042 + 0.23168402777777777778e-3 * t26715 + 0.23168402777777777778e-3 * t26718;
    (t27880, t27882, t27883, t27889)
}
