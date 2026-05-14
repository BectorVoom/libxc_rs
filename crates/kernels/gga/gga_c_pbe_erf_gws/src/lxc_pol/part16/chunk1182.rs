//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 16 (v4rho3sigma_4) CSE chunk 1182/1184 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part16_v4rho3sigma_4_chunk1182<F: Float>(t1211: F, t2429: F, t6926: F, t1172: F, t13763: F, t14364: F, t14368: F, t14831: F, t14852: F, t15101: F, t2074: F, t320: F, t3324: F, t3946: F, t4062: F, t4120: F, t52751: F, t52755: F, t52757: F, t52767: F, t52847: F, t54753: F, t54866: F, t54867: F, t54895: F, t54912: F, t54940: F, t54969: F, t55003: F, t55025: F, t55049: F, t55093: F, t55124: F, t55162: F, t55187: F, t55208: F, t55240: F, t55264: F, t55294: F, t55321: F, t55350: F, t55367: F, t55392: F, t55409: F, t55673: F, t55703: F, t55738: F, t55758: F, t55795: F, t55836: F, t55861: F, t55877: F, t55903: F, t55945: F, t55973: F, t55990: F, t944: F, t945: F) -> (F,) {
    let t56008 = 12.0 * t2429 * t1211 * t6926;
    let t56016 = -6.0 * t3946 * t15101 * t13763 - 3.0 * t3946 * t4120 * t52847 + t54866 - 2.0 * t4062 * t54867 * t944 - 2.0 * t4062 * t14364 * t3324 + t1172 * t320 * (t55003 + t55162 + t55990 + t55367 + t55861 + t55758 + t54912 + t55049 + t55321 + t55738 + t54940 + t55409 + t55187 + t55836 + t54969 + t54895 + t55392 + t55877 + t55025 + t55703 + t55264 + t55945 + t55973 + t55294 + t55673 + t55350 + t55240 + t55208 + t55795 + t55093 + t55903 + t55124) * t945 + 4.0 * t4062 * t52751 * t14831 + 2.0 * t52755 - 6.0 * t3946 * t4120 * t52767 + t56008 + 6.0 * t3946 * t14368 * t54753 + 6.0 * t52757 + 3.0 * t3946 * t14852 * t2074;
    (t56016,)
}
