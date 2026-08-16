//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 16 (v4rho3sigma_4) CSE chunk 1359/1361 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part16_v4rho3sigma_4_chunk1359(t1211: f64, t2429: f64, t6926: f64, t1172: f64, t13763: f64, t14364: f64, t14368: f64, t14831: f64, t14852: f64, t15101: f64, t2074: f64, t320: f64, t3324: f64, t3946: f64, t4062: f64, t4120: f64, t52751: f64, t52755: f64, t52757: f64, t52767: f64, t52847: f64, t54753: f64, t54866: f64, t54867: f64, t54895: f64, t54912: f64, t54940: f64, t54969: f64, t55003: f64, t55025: f64, t55049: f64, t55093: f64, t55124: f64, t55162: f64, t55187: f64, t55208: f64, t55240: f64, t55264: f64, t55294: f64, t55321: f64, t55350: f64, t55367: f64, t55392: f64, t55409: f64, t55673: f64, t55703: f64, t55738: f64, t55758: f64, t55795: f64, t55836: f64, t55861: f64, t55877: f64, t55903: f64, t55945: f64, t55973: f64, t55990: f64, t944: f64, t945: f64) -> f64 {
    let t56008 = 12.0_f64 * t2429 * t1211 * t6926;
    let t56016 = -6.0_f64 * t3946 * t15101 * t13763 - 3.0_f64 * t3946 * t4120 * t52847 + t54866 - 2.0_f64 * t4062 * t54867 * t944 - 2.0_f64 * t4062 * t14364 * t3324 + t1172 * t320 * (t55003 + t55162 + t55990 + t55367 + t55861 + t55758 + t54912 + t55049 + t55321 + t55738 + t54940 + t55409 + t55187 + t55836 + t54969 + t54895 + t55392 + t55877 + t55025 + t55703 + t55264 + t55945 + t55973 + t55294 + t55673 + t55350 + t55240 + t55208 + t55795 + t55093 + t55903 + t55124) * t945 + 4.0_f64 * t4062 * t52751 * t14831 + 2.0_f64 * t52755 - 6.0_f64 * t3946 * t4120 * t52767 + t56008 + 6.0_f64 * t3946 * t14368 * t54753 + 6.0_f64 * t52757 + 3.0_f64 * t3946 * t14852 * t2074;
    t56016
}
